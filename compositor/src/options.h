// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Yuxuan Shui <yshuiv7@gmail.com>
#pragma once

#include <stdbool.h>

struct c2_state;
struct options;
struct x_connection;

bool get_early_config(int argc, char *const *argv, char **config_file, bool *all_xerrors,
                      bool *fork, int *exit_code);
bool get_cfg(struct options *opt, int argc, char *const *argv);
void options_postprocess_c2_lists(struct c2_state *state, struct x_connection *c,
                                  struct options *option);
void options_destroy(struct options *options);
