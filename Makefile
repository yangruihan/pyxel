FORWARD_DIR = crates

.PHONY: noarg $(MAKECMDGOALS)

noarg:
	@$(MAKE) -C $(FORWARD_DIR)

$(MAKECMDGOALS):
	@$(MAKE) -C $(FORWARD_DIR) $@