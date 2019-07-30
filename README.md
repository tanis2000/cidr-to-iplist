# Convert a list of CIDR subnets into a list of IP addresses

This is a simple CLI tool to convert a list of network subnets in [CIDR format](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) into a list of IP addresses.

The only input is the filename containing the list of subnets separated by linefeeds.

An example of a list of subnets:

```txt
172.16.0.0/27
13.69.104.0/26
```

Here's how to run it:

```sh
cidr-to-iplist ./iplist.txt
```

The application will print the list of IP addresses to the standard output. If you need to save that in a file, just redirect it to the filename like this:

```sh
cidr-to-iplist ./iplist.txt > ips.txt
```
