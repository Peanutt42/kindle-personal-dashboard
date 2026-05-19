#pragma once

#include "kindle_personal_dashboard_core.h"
#include <gtk/gtk.h>
#include <string>

namespace kpd {

class Window {
  public:
	Window();

	void run();

	void update_automatic_screensaver_blocked_label(bool blocked);

  private:
	GtkWidget* window = nullptr;
	GtkWidget* quit_button = nullptr;
	std::string automatic_screensaver_blocked_text =
		_get_automatic_screensaver_blocked_text(
			core::kpd_is_automatic_screensaver_blocked()
		);
	GtkWidget* automatic_screensaver_blocked_label = nullptr;
	GtkWidget* toggle_automatic_screensaver_blocked_button = nullptr;

	static const char* _get_automatic_screensaver_blocked_text(bool blocked);
};

} // namespace kpd
