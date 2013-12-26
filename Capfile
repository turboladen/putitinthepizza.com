require 'bundler/capistrano'


load 'deploy' if respond_to?(:namespace) # cap2 differentiator

default_run_options[:pty] = true

# be sure to change these
set :github_user, 'turboladen'
set :github_application, "putitinthepizza.com"
set :user, 'putitinthepizza'
set :domain, 'putitinthepizza.com'
set :application, 'putitinthepizza'

# the rest should be good
set :repository,  "git@github.com:#{github_user}/#{github_application}.git"
set :deploy_to, "/var/www/#{domain}"
set :deploy_via, :remote_cache
#set :deploy_via, :export
set :scm, 'git'
set :branch, 'master'
set :scm_verbose, true
set :use_sudo, false

# My added options
ssh_options[:forward_agent] = true

server 'steveloveless.com', :app, :web

desc <<-DESC
A macro-task that updates the code, fixes the symlink, added the symlink
to the shared uploads folder. Finally takes a snapshot of the db.
DESC
deploy.task :default do
  transaction do
    update_code
    symlink
  end
end

# Override default tasks which are not relevant to a non-rails app.
namespace :deploy do
  #task :static do
  #  run "sh -c 'git clone --depth 1 git@github.com:turboladen/putitinthepizza.com.git /home2/stevehea/public_html/putitinthepizza && cd /home2/stevehea/public_html/putitinthepizza && git checkout -b deploy c323d06d14fcf49821cf629af68939e0814a1df8"
  #end

  task :migrate do
    puts "    not doing migrate because not a Rails application."
  end
  task :finalize_update do
    puts "    not doing finalize_update because not a Rails application."
  end
  task :start do
    puts "    not doing start because not a Rails application."
  end
  task :stop do
    puts "    not doing stop because not a Rails application."
  end
  task :restart do
    puts "    not doing restart because not a Rails application."
  end
end
