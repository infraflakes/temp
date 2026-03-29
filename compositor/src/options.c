// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Yuxuan Shui <yshuiv7@gmail.com>
// Debloated for embedded use — CLI options handled by Lua config

#include <getopt.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "log.h"
#include "options.h"
#include "config.h"

enum {
	OPT_CONFIG = 256,
};

static const struct option longopts[] = {
	{"config", required_argument, NULL, OPT_CONFIG},
	{"help", no_argument, NULL, 'h'},
	{NULL, 0, NULL, 0},
};

static const char *shortopts = "hc:";

static void usage(const char *argv0) {
	fprintf(stdout, "srcom (embedded compositor)\n");
	fprintf(stdout, "Usage: %s --config <path>\n", argv0);
	fprintf(stdout, "  -c, --config <path>  Path to configuration file\n");
	fprintf(stdout, "  -h, --help           Show this help\n");
}

bool get_early_config(int argc, char *const *argv, char **config_file, bool *all_xerrors,
                      bool *fork, int *exit_code) {
	int opt;
	*config_file = NULL;
	*exit_code = 0;

	while ((opt = getopt_long(argc, argv, shortopts, longopts, NULL)) != -1) {
		switch (opt) {
		case OPT_CONFIG:
		case 'c':
			*config_file = strdup(optarg);
			break;
		case 'h':
			usage(argv[0]);
			return true;
		case '?':
			return true;
		default:
			break;
		}
	}

	return false;
}

bool get_cfg(options_t *opt, int argc, char *const *argv) {
	return true;
}

void options_postprocess_c2_lists(struct c2_state *state, struct x_connection *c,
                                  struct options *option) {
}

void options_destroy(struct options *options) {
}