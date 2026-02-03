watch +args='ltest':
  cargo watch --clear --exec '{{ args }}'

python:
  ./bin/python -c 'print("Hello, world!")'
