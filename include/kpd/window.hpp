#pragma once

#include "gh_heatmap.hpp"
#include "kindle_personal_dashboard_core.h"

#include <gtk/gtk.h>

#include <memory>
#include <string>

namespace kpd {

class Window {
  public:
	Window(std::shared_ptr<core::State> core_state);

	void run();

	void update_automatic_screensaver_blocked_label(bool blocked);

  private:
	GtkWidget* window = nullptr;
	GtkWidget* quit_button = nullptr;
	std::string automatic_screensaver_blocked_text =
		_get_automatic_screensaver_blocked_text(
			core::kpd_core_is_automatic_screensaver_blocked()
		);
	GtkWidget* automatic_screensaver_blocked_label = nullptr;
	GtkWidget* toggle_automatic_screensaver_blocked_button = nullptr;

	GHHeatmap gh_heatmap;

	static const char* _get_automatic_screensaver_blocked_text(bool blocked);

	static void _on_window_configure(
		GtkWidget* window,
		GdkEvent* event,
		gpointer user_data
	);
};

} // namespace kpd
