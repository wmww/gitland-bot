#!/usr/bin/python3

import logging
import os
import stat
import shutil
from os import system, path
from os.path import realpath, dirname, expanduser

logger = logging.getLogger(__name__)

required_actions = []
def add_required_action(name, action):
    global required_actions
    logger.info('User will need to ' + name)
    required_actions.append((name, action))

def show_required_actions():
    global required_actions
    if not required_actions:
        logger.info('No required user actions')
        return
    logger.info('Showing required user actions')
    print()
    print('You now must do the following:')
    for name, action in required_actions:
        print('# ' + name)
        print()
        print(action)
        print()

def run_command(cmd):
    logger.info('Running `' + cmd + '`')
    if system(cmd) != 0:
        raise RuntimeError('`' + cmd + '` failed')

def git_clone(repo_url, dst_path):
    run_command('git clone "' + repo_url + '" "' + dst_path + '"')

def git_pull(repo_path):
    logger.info('Pulling ' + repo_path)
    run_command('git -C "' + repo_path + '" pull')

def git_add_remote(repo_path, name, url):
    logger.info('Adding ' + name + ' remote to ' + repo_path)
    run_command('git -C "' + repo_path + '" remote add ' + name + ' ' + url)

def fix_path(p):
    '''Takes a path relative to the bot repo dir and returns an absolute path'''
    p = expanduser(p)
    if path.isabs(p):
        return realpath(p)
    else:
        return realpath(dirname(__file__) + '/' + p)

class Context:
    def __init__(self):
        self.github_name = 'wmww'
        self.git_email = 'wm@wmww.sh'
        self.git_name = 'wmww\'s gitland bot'
        self.required_ubuntu_packages = ['git', 'cargo', 'pkg-config', 'libssl-dev']
        self.bot_repo_path = fix_path('.')
        self.client_repo_name = 'gitland-client'
        self.client_repo_path = fix_path('../' + self.client_repo_name)
        self.server_repo_path = fix_path('../gitland')
        self.ssh_dir_path = fix_path('~/.ssh')
        self.ssh_config_path = self.ssh_dir_path + '/config'
        self.ssh_key_name = 'gitland-client.deploy'
        self.ssh_pub_key_path = self.ssh_dir_path + '/' + self.ssh_key_name + '.pub'
        self.ssh_priv_key_path = self.ssh_dir_path + '/' + self.ssh_key_name
        self.runner_bin_path = fix_path('/usr/bin/gitland-bot')
        self.service_file_dst = fix_path('/etc/systemd/system/gitland-bot.service')

    def update_and_install(self):
        run_command('apt update')
        run_command('apt upgrade')
        run_command('apt install ' + ' '.join(self.required_ubuntu_packages))

    def setup_git(self):
        git_conf_path = fix_path('~/.gitconfig')
        if path.exists(git_conf_path):
            logging.info(git_conf_path + ' exists, so assuming git is already configured')
            return
        run_command('git config --global user.email "' + self.git_email + '"')
        run_command('git config --global user.name "' + self.git_name + '"')

    def setup_gitland_client_repo(self):
        if path.exists(self.client_repo_path):
            logger.info(self.client_repo_path + ' exists, assuming client repo set up correctly')
        else:
            git_clone(
                'https://github.com/' + self.github_name + '/gitland-client.git',
                self.client_repo_path)
            git_add_remote(
                self.client_repo_path,
                'deploy',
                'git@github.com:' + self.github_name + '/gitland-client.git')
        git_pull(self.client_repo_path)
    
    def setup_gitland_server_repo(self):
        if path.exists(self.server_repo_path):
            logger.info(self.server_repo_path + ' exists, assuming server repo set up correctly')
        else:
            git_clone(
                'https://github.com/programical/gitland.git',
                self.server_repo_path)
        # Pull happens every cycle anyway

    def _create_deploy_key_pair(self):
        logger.info('Creating SSH key pair to use as GitHub deploy keys')
        run_command('ssh-keygen -t rsa -b 4096 -C wm@wmww.sh -f ' + self.ssh_priv_key_path + ' -P ""')
        if not path.exists(self.ssh_priv_key_path):
            raise RuntimeError('Private key not created at ' + self.ssh_priv_key_path)
        if not path.exists(self.ssh_pub_key_path):
            raise RuntimeError('Public key not created at ' + self.ssh_pub_key_path)
        pub_key_file = open(self.ssh_pub_key_path)
        pub_key_contents = pub_key_file.read()
        pub_key_file.close()
        add_required_action(
            'Add deploy key to gitland-client GitHub repo',
            'For the bot to be able to push to GitHub, you must add the new deploy key to the GitHub repo.\n' +
            '- Go to https://github.com/' + self.github_name + '/gitland-client/settings/keys/new\n' +
            '- Enter "Gitland bot server" for the title\n' +
            '- Paste the following into they key box:\n' +
            '\n' + pub_key_contents + '\n' +
            '- Enable "Allow write access"\n' +
            '- Add key')

    def _add_deploy_keys_to_config(self):
        if not path.exists(self.ssh_config_path):
            logger.info('Creating SSH config directory')
            open(self.ssh_config_path, 'a').close()
        host_line = 'host github.com'
        config = open(self.ssh_config_path, 'r')
        for line in config.readlines():
            if line.strip() == host_line:
                logger.log('SSH config already has ' + self.client_repo_name + ' host')
                return
        config.close()
        config = open(self.ssh_config_path, 'a')
        config.write('\n' +
            host_line +'\n' +
            'user git\n' +
            'identityfile ' + self.ssh_priv_key_path + '\n')
        config.close()

    def setup_deploy_key(self):
        if path.exists(self.ssh_priv_key_path):
            logger.info('SSH deploy key already set up')
            return
        if not path.exists(self.ssh_dir_path):
            logger.info('Making ' + self.ssh_dir_path)
            os.mkdir(self.ssh_dir_path)
        self._create_deploy_key_pair()
        self._add_deploy_keys_to_config()

    def setup_gitland_bot_runner(self):
        logger.info('Writing runner script to ' + self.runner_bin_path)
        contents = (
            '#!/bin/bash\n' +
            'set -euo pipefail\n' +
            'cd ' + self.bot_repo_path + '\n' +
            './run_script.sh\n')
        runner_file = open(self.runner_bin_path, 'w')
        runner_file.write(contents)
        runner_file.close()
        st = os.stat(self.runner_bin_path)
        os.chmod(self.runner_bin_path, st.st_mode | stat.S_IEXEC)

    def setup_systemd_service(self):
        service_file = fix_path('gitland-bot.service')
        logging.info('Copying service file at ' + service_file + ' to ' + self.service_file_dst)
        shutil.copy(service_file, self.service_file_dst)
        add_required_action(
            'Enable systemd service',
            'Run `systemctl enable gitland-bot` ' +
            'and/or `systemctl start gitland-bot` ' +
            'to run the bot')

logging.basicConfig(level=logging.DEBUG)
context = Context()
context.update_and_install()
context.setup_git()
context.setup_gitland_client_repo()
context.setup_gitland_server_repo()
context.setup_deploy_key()
context.setup_gitland_bot_runner()
context.setup_systemd_servicec()
show_required_actions()
logger.info('Done')
