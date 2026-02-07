# Shell to use for executing commands
SHELL=	/bin/sh

# Path to the ed binary being tested
ED=	${.OBJDIR}/ed

# Default target that runs all checks
all: check
	@:

# Target to run both build and test, and report success if no errors are found
check: build test
	@if grep -h '\*\*\*' errs.o scripts.o; then :; else \
		echo "tests completed successfully."; \
	fi

# Target to build the test scripts if they do not already exist
build: mkscripts.sh
	@if [ ! -f errs.o ]; then \
		uudecode < ascii.d.uu ; \
		uudecode < ascii.r.uu ; \
		echo "building test scripts for $(ED) ..."; \
		$(SHELL) mkscripts.sh $(ED); \
	fi

# Target to run the test scripts
test: build ckscripts.sh
	@echo testing $(ED) ...
	@$(SHELL) ckscripts.sh $(ED)

# Clean target to remove generated files
clean:
	rm -f *.ed *.red *.[oz] *~ ascii.d ascii.r
