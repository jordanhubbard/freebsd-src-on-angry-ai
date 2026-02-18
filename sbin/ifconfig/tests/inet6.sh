# $FreeBSD$
#
# Copyright (c) 2025 The FreeBSD Foundation
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions
# are met:
# 1. Redistributions of source code must retain the above copyright
#    notice, this list of conditions and the following disclaimer.
# 2. Redistributions in binary form must reproduce the above copyright
#    notice, this list of conditions and the following disclaimer in the
#    documentation and/or other materials provided with the distribution.
#
# THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
# ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
# ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
# OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
# HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
# LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
# OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
# SUCH DAMAGE.

. $(atf_get_srcdir)/../../common/vnet.subr

# Bug 286910: adding 'netmask' or 'broadcast' to an IPv6 address crashed
# ifconfig.

atf_test_case "netmask" "cleanup"
netmask_head()
{
	atf_set descr "Test invalid 'netmask' option"
	atf_set require.user root
}

netmask_body()
{
	vnet_init

	ep=$(vnet_mkepair)
	vnet_mkjail ifcjail ${ep}a

	# Add the address the wrong way
	atf_check -s exit:1 \
	    -e match:"ifconfig: netmask: invalid option for inet6" \
	    jexec ifcjail ifconfig ${ep}a inet6 2001:db8:1::1 netmask 64

	# Add the address the correct way
	atf_check -s exit:0 \
	    jexec ifcjail ifconfig ${ep}a inet6 2001:db8:1::1/64
	atf_check -s exit:0 -o match:"2001:db8:1::1 prefixlen 64" \
	    jexec ifcjail ifconfig ${ep}a

	# Remove the address the wrong way
	atf_check -s exit:1 \
	    -e match:"ifconfig: netmask: invalid option for inet6" \
	    jexec ifcjail ifconfig ${ep}a inet6 2001:db8:1::1 netmask 64 -alias
}

netmask_cleanup()
{
	vnet_cleanup
}

atf_test_case "broadcast" "cleanup"
broadcast_head()
{
	atf_set descr "Test invalid 'broadcast' option"
	atf_set require.user root
}

broadcast_body()
{
	vnet_init

	ep=$(vnet_mkepair)
	vnet_mkjail ifcjail ${ep}a

	atf_check -s exit:1 \
	    -e match:"ifconfig: broadcast: invalid option for inet6" \
	    jexec ifcjail ifconfig ${ep}a \
	        inet6 2001:db8:1::1 broadcast 2001:db8:1::ffff

	atf_check -s exit:0 \
	    jexec ifcjail ifconfig ${ep}a inet6 2001:db8:1::1/64

	atf_check -s exit:1 \
	    -e match:"ifconfig: broadcast: invalid option for inet6" \
	    jexec ifcjail ifconfig ${ep}a \
	        inet6 2001:db8:1::1 broadcast 2001:db:1::ffff -alias
}

broadcast_cleanup()
{
	vnet_cleanup
}

atf_test_case "delete6" "cleanup"
delete6_head()
{
	atf_set descr 'Test removing IPv6 addresses'
	atf_set require.user root
}

delete6_body()
{
	vnet_init

	ep=$(vnet_mkepair)

	atf_check -s exit:0 \
	    ifconfig ${ep}a inet6 fe80::42/64
	atf_check -s exit:0 -o match:"fe80::42%${ep}" \
	    ifconfig ${ep}a inet6

	atf_check -s exit:0 \
	    ifconfig ${ep}a inet6 -alias fe80::42
	atf_check -s exit:0 -o not-match:"fe80::42%${ep}" \
	    ifconfig ${ep}a inet6
}

delete6_cleanup()
{
	vnet_cleanup
}

atf_init_test_cases()
{
	atf_add_test_case netmask
	atf_add_test_case broadcast
	atf_add_test_case delete6
}
