# frozen_string_literal: true

require "test/unit"
require_relative "../lib/rweb3"

class KeyTest < Test::Unit::TestCase

  def test_new_key
    key = RWeb3::Key.new
    pp key
  end

end
