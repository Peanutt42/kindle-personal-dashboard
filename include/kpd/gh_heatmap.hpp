#pragma once

#include "kindle_personal_dashboard_core.h"
#include <gtk/gtk.h>
#include <memory>

namespace kpd {
class GHHeatmap {
  public:
	GHHeatmap(std::shared_ptr<core::State> state);

	void configure_layout(GdkWindow* window);

	GtkWidget* get_widget() { return this->drawing_area; }

  private:
	std::shared_ptr<core::State> core_state;
	GtkWidget* drawing_area = nullptr;

	constexpr static size_t PADDING = 4;
	constexpr static uint8_t MAX_CONTRIBUTION_LEVEL = 4;

	static gboolean _drawing_area_on_expose(
		GtkWidget* widget,
		GdkEventExpose* event,
		gpointer user_data
	);
};
} // namespace kpd
