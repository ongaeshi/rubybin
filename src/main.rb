require 'open-uri'

ARGV.each do |url|
  URI.open(url) {|f|
    f.each_line {|line| puts line}
  }
end