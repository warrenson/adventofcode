#!/bin/bash
awk 'BEGIN{sum=0} {sum += int($1/3) - 2} END{print sum}'
