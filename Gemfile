source 'https://rubygems.org'

gem 'sinatra', '1.4.7'

group :production do
  gem 'passenger', '>= 5.0.25', require: 'phusion_passenger/rack_handler'
end

group :development do
  gem 'capistrano', '~> 3.5'
  gem 'capistrano-chruby'
  gem 'capistrano-bundler'
  gem 'capistrano-passenger'
  gem 'highline'
  gem 'puma', '~> 2.7.1'
end
