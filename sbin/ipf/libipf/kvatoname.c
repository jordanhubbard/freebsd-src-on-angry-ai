
/*
 * Copyright (C) 2012 by Darren Reed.
 *
 * See the IPFILTER.LICENCE file for details on licencing.
 *
 * $Id$
 */

#include "ipf.h"

#include <fcntl.h>
#include <sys/ioctl.h>

char *
kvatoname(ipfunc_t func, ioctlfunc_t iocfunc)
{
	static char funcname[40];
	ipfunc_resolve_t res;
	int fd;

	res.ipfu_addr = func;
	res.ipfu_name[0] = '\0';
	fd = -1;

	if ((opts & OPT_DONTOPEN) == 0) {
		fd = open(IPL_NAME, O_RDONLY);
		if (fd == -1)
			return (NULL);
	}
	if ((opts & OPT_DONTOPEN) == 0) {
		fd = open(IPL_NAME, O_RDONLY);
		if (fd == -1)
			return (NULL);
	}
	if ((*iocfunc)(fd, SIOCFUNCL, &res) != 0) {
		close(fd);
		return (NULL);
	}
	if (fd >= 0)
		close(fd);
	strlcpy(funcname, res.ipfu_name, sizeof(funcname));
	return (funcname);
}
