#include "window.hpp"
#include "g_signal_lambda.hpp"
#include "kindle_personal_dashboard_core.h"

#include <gdk/gdk.h>
#include <glib-object.h>
#include <gtk/gtk.h>

#ifndef KPD_LOCAL_DEV
#include "kindle_awm_helper.hpp"
#endif

namespace kpd {

Window::Window() {
	// NOLINTNEXTLINE(cppcoreguidelines-prefer-member-initializer)
	window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

#ifdef KPD_LOCAL_DEV
	constexpr static int LOCAL_DEV_WIDTH = 1024;
	constexpr static int LOCAL_DEV_HEIGHT = 758;

	gtk_window_set_title(
		GTK_WINDOW(window), "Kindle Personal Dashboard [LOCAL DEV]"
	);
	gtk_window_set_default_size(
		GTK_WINDOW(window), LOCAL_DEV_WIDTH, LOCAL_DEV_HEIGHT
	);
	gtk_window_set_resizable(GTK_WINDOW(window), FALSE);
	gtk_window_set_decorated(GTK_WINDOW(window), FALSE);
	gtk_window_set_type_hint(GTK_WINDOW(window), GDK_WINDOW_TYPE_HINT_DIALOG);

	// set minimum size
	GdkGeometry geometry;
	memset(&geometry, 0, sizeof geometry);
	geometry.min_width = LOCAL_DEV_WIDTH;
	geometry.min_height = LOCAL_DEV_HEIGHT;
	geometry.max_width = LOCAL_DEV_WIDTH;
	geometry.max_height = LOCAL_DEV_HEIGHT;

	// NOLINTNEXTLINE(clang-analyzer-optin.core.EnumCastOutOfRange)
	constexpr static auto MIN_MAX_SIZE_HINTS = static_cast<GdkWindowHints>(
		GdkWindowHints::GDK_HINT_MIN_SIZE | GdkWindowHints::GDK_HINT_MAX_SIZE
	);

	gtk_window_set_geometry_hints(
		GTK_WINDOW(window), nullptr, &geometry, MIN_MAX_SIZE_HINTS
	);
#else
	gtk_window_set_title(
		GTK_WINDOW(window),
		KINDLE_AWM_BUILD_TITLE(
			KINDLE_AWM_LAYER_APP, KINDLE_AWM_NAME_APPLICATION,
			"com.github.kindle-personal-dashboard", KINDLE_AWM_PC_TOPBAR_STATUS,
			KINDLE_AWM_ORIENTATION_LEFT
		)
	);
#endif

	g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), nullptr);

	GtkWidget* vbox = gtk_vbox_new(FALSE, 5);
	gtk_container_set_border_width(GTK_CONTAINER(vbox), 10);
	gtk_container_add(GTK_CONTAINER(window), vbox);

	// NOLINTBEGIN(cppcoreguidelines-prefer-member-initializer)
	quit_button = gtk_button_new_with_label("Quit");
	automatic_screensaver_blocked_label =
		gtk_label_new(automatic_screensaver_blocked_text.c_str());
	toggle_automatic_screensaver_blocked_button =
		gtk_button_new_with_label("Toggle Automatic Screensaver Blocking");
	// NOLINTEND(cppcoreguidelines-prefer-member-initializer)

	gtk_box_pack_start(GTK_BOX(vbox), quit_button, TRUE, TRUE, 0);
	gtk_box_pack_start(
		GTK_BOX(vbox), automatic_screensaver_blocked_label, TRUE, TRUE, 0
	);
	gtk_box_pack_start(
		GTK_BOX(vbox), toggle_automatic_screensaver_blocked_button, TRUE, TRUE,
		0
	);

	g_signal_connect_clicked_lambda(quit_button, []() { gtk_main_quit(); });

	g_signal_connect_clicked_lambda(
		toggle_automatic_screensaver_blocked_button, [this]() {
			const bool blocked = core::kpd_is_automatic_screensaver_blocked();
			core::kpd_set_automatic_screensaver_blocked(!blocked);

			this->update_automatic_screensaver_blocked_label(!blocked);
		}
	);
}

void Window::run() {
	gtk_widget_show_all(window);

	gtk_main();
}

void Window::update_automatic_screensaver_blocked_label(bool blocked) {
	gtk_label_set_label(GTK_LABEL(automatic_screensaver_blocked_label), "");
	automatic_screensaver_blocked_text =
		_get_automatic_screensaver_blocked_text(blocked);
	gtk_label_set_label(
		GTK_LABEL(automatic_screensaver_blocked_label),
		automatic_screensaver_blocked_text.c_str()
	);
}

const char* Window::_get_automatic_screensaver_blocked_text(bool blocked) {
	return blocked ? "Automatic Screensaver is blocked"
				   : "Automatic Screensaver is not blocked";
}

} // namespace kpd
