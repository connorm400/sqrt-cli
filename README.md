# sqrt-cli

this is a commandline application that will find the precise sqare root of a number
to any number of digits of accuracy. Doesn't use the standard libraries's sqrt function
just newton's method

## how to use it

the number to square root should be piped in throught standard input. Set the # of digits 
of accuracy with the `-a` flag.

```sh
$ echo "2" | sqrt-cli -a 10
1.4142135623746899
```