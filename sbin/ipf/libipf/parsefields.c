#include "ipf.h"
#include <err.h>

extern int nohdrfields;

wordtab_t *parsefields(wordtab_t *table, char *arg)
{
	wordtab_t *f, *fields;
	char *s, *t;
	int num;

	fields = NULL;
	num = 0;

	for (s = strtok(arg, ","); s != NULL; s = strtok(NULL, ",")) {
		t = strchr(s, '=');
		if (t != NULL) {
			*t++ = '\0';
			if (*t == '\0') {
				nohdrfields = 1;
				continue;
			}
		}

		f = findword(table, s);
		if (f == NULL) {
			warnx("Unknown field '%s'", s);
			free(fields);
			return (NULL);
		}

		num++;
		if (fields == NULL) {
			fields = malloc(2 * sizeof(*fields));
		} else {
			fields = reallocarray(fields, num + 1, sizeof(*fields));
			if (fields == NULL) {
				warnx("memory allocation error");
				free(fields);
				return (NULL);
			}
		}

		if (t == NULL) {
			fields[num - 1].w_word = f->w_word;
		} else {
			fields[num - 1].w_word = strdup(t);
			if (fields[num - 1].w_word == NULL) {
				warnx("memory allocation error");
				free(fields);
				return (NULL);
			}
		}
		fields[num - 1].w_value = f->w_value;
		fields[num].w_word = NULL;
		fields[num].w_value = 0;
	}

	return (fields);
}
