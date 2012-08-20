load 'deploy' if respond_to?(:namespace) # cap2 differentiator

default_run_options[:pty] = true

# be sure to change these
set :github_user, 'turboladen'
set :github_application, "putitinthepizza.com"
set :user, 'stevehea'
set :domain, 'steveloveless.com'
set :application, 'putitinthepizza'

# the rest should be good
set :repository,  "git@github.com:#{github_user}/#{github_application}.git"
set :deploy_to, "/home2/stevehea/public_html/#{application}"  # or whatever path you want to copy it to
set :deploy_via, :remote_cache
set :scm, 'git'
set :branch, 'master'
set :git_shallow_clone, 1
set :scm_verbose, true
set :use_sudo, false

server domain, :app, :web

