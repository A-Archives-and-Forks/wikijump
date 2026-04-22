#!/bin/sh
curl \
	-If \
	-H 'x-wikijump-site-slug: www' \
	-H 'x-wikijump-site-id: 6000000' \
	http://localhost:3393/
