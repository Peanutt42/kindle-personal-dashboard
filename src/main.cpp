#include <gtk/gtk.h>

#include "kindle_awm_helper.hpp"

#include "kindle_personal_dashboard_core.h"

constexpr static const char* KDP_AWM_TITLE = KINDLE_AWM_TITLE(
	KINDLE_AWM_LAYER_APP,
	KINDLE_AWM_NAME_APPLICATION,
	"com.github.kindle-personal-dashboard",
	KINDLE_AWM_PC_TOPBAR_STATUS
);

static void
on_clicked([[maybe_unused]] GtkWidget* widget, [[maybe_unused]] gpointer data) {
	gtk_main_quit();
}

int main(int argc, char* argv[]) {
	GtkWidget* window = nullptr;
	GtkWidget* button = nullptr;

	kindle_personal_dashboard_core_foo();

	gtk_init(&argc, &argv);

	window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
	gtk_window_set_title(GTK_WINDOW(window), KDP_AWM_TITLE);

	button = gtk_button_new_with_label("Click Me");

	g_signal_connect(button, "clicked", G_CALLBACK(on_clicked), nullptr);
	g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), nullptr);

	gtk_container_add(GTK_CONTAINER(window), button);
	gtk_widget_show_all(window);
	gtk_main();

	return 0;
}
