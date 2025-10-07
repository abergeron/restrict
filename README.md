Small command to limit what can be run using ssh key authentification.

Usage
-----

In authorized_keys
```
restrict,command="restrict /path/to/rules" AAAB...Xqv
```

The rules file
```
# comments are allowed

# you can have exact matches
cat /var/log/messages
# or use regex matching
ls /home/.*
```
See https://docs.rs/regex/latest/regex/#syntax for an explanation of the supported syntax.

Any error that happens during rules parsing or matching will lead to a `denied` message and some more information in the logs at the error level.

Testing
-------

To test if a rule file does what you want you can do this

`SSH_ORIGINAL_COMMAND="test command --args" restrict /path/to/rules`

The rule that matched the command will be shown in the logs at the info level.
