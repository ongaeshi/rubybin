require 'open-uri'
URI.open(ARGV[0]) {|f|
  f.each_line {|line| puts line}
}