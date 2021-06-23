require 'open-uri'
URI.open("http://www.ruby-lang.org/") {|f|
  f.each_line {|line| puts line}
}