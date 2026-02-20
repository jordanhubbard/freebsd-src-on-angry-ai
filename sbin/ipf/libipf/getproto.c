
/*
 * Copyright (C) 2012 by Darren Reed.
 *
 * See the IPFILTER.LICENCE file for details on licencing.
 *
 * $Id$
 */

#include "ipf.h"
#include <ctype.h>

int getproto(char *name);

int
getproto(char *name)
{
	struct protoent *p;
	char *s;

	for (s = name; *s != '\0'; s++)
		if (!ISDIGIT(*s))
			break;
	if (*s == '\0') {
		long proto_num;
		char *endptr;

		errno = 0;
		proto_num = strtol(name, &endptr, 10);
		if (errno != 0 || *endptr != '\0' || proto_num < 0 || proto_num > INT_MAX)
			return (-1);
		return ((int)proto_num);
	}

	if (!strcasecmp(name, "ip"))
		return (IPPROTO_IP);

	p = getprotobyname(name);
	if (p != NULL)
		return (p->p_proto);
	return (-1);
}
