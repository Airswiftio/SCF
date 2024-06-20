default: build

build:
	$(MAKE) -C ./scf_soroban || break;
	$(MAKE) -C ./contract_deployer || break;
	$(MAKE) -C ./scf_pool || break;
	$(MAKE) -C ./argentina_pledge || break;
	$(MAKE) -C ./argentina_pool || break;

testall:
	$(MAKE) -C ./scf_soroban test || break;
	$(MAKE) -C ./contract_deployer test || break;
	$(MAKE) -C ./scf_pool test || break;
	$(MAKE) -C ./argentina_pledge test || break;
	$(MAKE) -C ./argentina_pool test || break;