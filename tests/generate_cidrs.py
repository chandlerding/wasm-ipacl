#!/usr/bin/env python3

import random
import ipaddress
import argparse

def generate_random_ipv4_cidr(min_prefix=16):
    """Generates a random IPv4 CIDR."""
    # Generate a random 32-bit integer for the IP address
    random_ip_int = random.randint(0, 2**32 - 1)
    ip = ipaddress.IPv4Address(random_ip_int)

    # Generate a random prefix length
    prefix_len = random.randint(min_prefix, 32)

    # Create the network and return the CIDR string
    network = ipaddress.IPv4Network(f"{ip}/{prefix_len}", strict=False)
    return network.with_prefixlen

def generate_random_ipv6_cidr(min_prefix=48):
    """Generates a random IPv6 CIDR."""
    # Generate a random 128-bit integer for the IP address
    random_ip_int = random.randint(0, 2**128 - 1)
    ip = ipaddress.IPv6Address(random_ip_int)

    # Generate a random prefix length
    prefix_len = random.randint(min_prefix, 128)

    # Create the network and return the CIDR string
    network = ipaddress.IPv6Network(f"{ip}/{prefix_len}", strict=False)
    return network.with_prefixlen

def main():
    parser = argparse.ArgumentParser(description="Generate a list of random CIDRs.")
    parser.add_argument("count", type=int, help="Number of CIDRs to generate.")
    args = parser.parse_args()

    for i in range(args.count):
        # Randomly choose between generating an IPv4 or IPv6 CIDR
        if random.choice([True, False]):
            print(generate_random_ipv4_cidr())
        else:
            print(generate_random_ipv6_cidr())

if __name__ == "__main__":
    main()