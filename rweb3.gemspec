# frozen_string_literal: true

require_relative "lib/rweb3/version"

Gem::Specification.new do |spec|
  spec.name = "rweb3"
  spec.version = RWeb3::VERSION
  spec.authors = ["Peter Chung"]
  spec.email = ["touhonoob@gmail.com"]

  spec.summary = "rust-web3 bindings for Ruby"
  spec.description = "A Ruby binding for rust-web3."
  spec.homepage = "https://github.com/ruby-web3/rweb3"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.5"

  spec.files = Dir["{lib,ext}/**/*", "LICENSE", "README.md", "Cargo.*"]
  spec.files.reject! { |f| File.directory?(f) }
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.extensions = ["ext/extconf.rb"] # Future: ["ext/Cargo.toml"] with rubygems >= 3.3.24

  spec.rdoc_options += ["--exclude", "vendor"]

  # Can be removed for binary gems and rubygems >= 3.3.24
  spec.add_dependency "rb_sys", "~> 0.9.46"
end