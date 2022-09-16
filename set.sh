#!/bin/bash

f=$1

if [ ! -f "$f" ]; then
	>&2 echo $f is not a file
	exit 1
fi

echo -n "Title: "
read title
if [ ! -z "$title" ]; then
	setfattr -n user.dcterms:title -v "$title" "$f"
fi

getfattr -n user.dcterms:title "$f" &> /dev/null
if [ "$?" -gt "0" ]; then
	>&2 echo no title set, exiting.
	exit 0
fi

echo -n "Author: "
read author
if [ ! -z "$author" ]; then
	setfattr -n user.dcterms:creator -v "$author" "$f"
fi

echo -n "Subject (comma,separated list): "
read subject
if [ ! -z "$subject" ]; then
	setfattr -n user.dcterms:subject -v "$subject" "$f"
fi

echo -n "Language: "
read language
if [ ! -z "$language" ]; then
	setfattr -n user.dcterms:language -v "$language" "$f"
fi

echo -n "Type: "
read typ
if [ ! -z "$typ" ]; then
	setfattr -n user.dcterms:type -v "$typ" "$f"
fi

mime=`file -b --mime-type "$f"`
echo -n "Mime ($mime): "
read mime_in
if [ ! -z "$mime_in" ]; then
	mime=$mime_in
fi 
setfattr -n user.dcterms:MediaType -v "$mime" "$f"
