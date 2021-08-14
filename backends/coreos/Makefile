all: run

docker-host.ign: docker-host.bu
	butane --pretty --strict $< > $@

# FIXME: this was super-slow last time.
# Maybe increase the ram/disk like we did with fedora?
.PHONY: run
run: docker-host.ign
	qemu-system-x86_64 -m 2048 -nographic -snapshot \
		-drive if=virtio,file=$(HOME)/Downloads/fedora-coreos-34.20210725.3.0-qemu.x86_64.qcow2 \
		-fw_cfg name=opt/com.coreos/config,file=coreos/docker-host.ign \
		-nic user,model=virtio,hostfwd=tcp::2222-:22
