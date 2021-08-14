all: run

debian-10-openstack-amd64.qcow2: $(HOME)/Downloads/debian-10-openstack-amd64.qcow2
	qemu-img create \
		-f qcow2 \
		-b $(HOME)/Downloads/debian-10-openstack-amd64.qcow2 \
		debian-10-openstack-amd64.qcow2 \
		30G

cidata.iso: user-data meta-data
	mkisofs -output cidata.iso -V cidata -r -J user-data meta-data

# FIXME: this plays havoc with your shell 
.PHONY: run
run: debian-10-openstack-amd64.qcow2 cidata.iso
	qemu-system-x86_64 -m 8192 -nographic \
		-drive if=virtio,file=debian-10-openstack-amd64.qcow2 \
		-cdrom cidata.iso \
		-nic user,model=virtio,hostfwd=tcp::2222-:22
