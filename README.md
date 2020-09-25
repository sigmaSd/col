# col
`awk '{print $x}'` replacement

# Usage example
`printf 'hello world\nhello2 world2' | col 1` => 'hello\nhello2'

`printf 'hello world\nhello2 world2' | col -1` => 'world\nworld2'
