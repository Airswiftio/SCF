default: build

build:
	$(MAKE) -C ./scf_soroban || break;
	$(MAKE) -C ./token || break;
	$(MAKE) -C ./contract_deployer || break;
	$(MAKE) -C ./scf_pool || break;