set remotetimeout 10
# set debug remote 1 # if there are issues;
target extended-remote :3333
monitor reset halt
maintenance flush register-cache
b main
continue
layout src
