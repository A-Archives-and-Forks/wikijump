#!/bin/sh
curl \
	-If \
	-H 'x-wikijump-site-slug: www' \
	-H 'x-wikijump-site-id: 1' \
	http://localhost:3000/
