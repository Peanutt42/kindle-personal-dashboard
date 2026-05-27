#include "kindle_personal_dashboard_core.h"

#include "window.hpp"

#include <memory>

using namespace kpd;

int main(int argc, char* argv[]) {
	gtk_init(&argc, &argv);

	std::shared_ptr<core::State> core_state(
		core::kpd_core_state_new(), &core::kpd_core_state_delete
	);

	Window window(std::move(core_state));

	window.run();

	return 0;
}
