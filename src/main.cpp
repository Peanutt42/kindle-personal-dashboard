#include "kindle_personal_dashboard_core.h"

#include "window.hpp"

using namespace kpd;

int main(int argc, char* argv[]) {
	gtk_init(&argc, &argv);

	core::State* rust_state = core::kpd_state_new();

	Window window;

	window.run();

	core::kpd_state_delete(rust_state);

	return 0;
}
