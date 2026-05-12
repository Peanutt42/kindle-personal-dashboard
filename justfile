
# configure cmake with preset
configure preset="local":
	cmake --preset={{preset}}

# compile with preset
build preset="local":
	cmake --build --preset={{preset}} -j

kindle_personal_dashboard_binary := "./build/kindle_personal_dashboard"

# runs locally on host machine
run preset="local":
	@echo "Building kindle_personal_dashboard..."
	cmake --build --preset={{preset}} -j

	@echo ""
	@echo "Running kindle_personal_dashboard..."
	{{if preset == "local" {
		"./build/kindle_personal_dashboard"
	} else if preset == "local_release" {
		"./build_release/kindle_personal_dashboard"
	} else {
		"./build_" + preset + "/kindle_personal_dashboard"
	}}}
	

# compiles and uploads binary to kindle over ssh
upload kindle_ip kindle_type="kindlepw2":
	@echo "Building kindle_personal_dashboard for {{kindle_type}}..."
	cmake --build --preset={{kindle_type}} -j

	@echo ""
	@echo "Copying kindle_personal_dashboard over to {{kindle_type}} (root@{{kindle_ip}})..."
	scp ./build_{{kindle_type}}/kindle_personal_dashboard root@{{kindle_ip}}:/mnt/us/extensions/
