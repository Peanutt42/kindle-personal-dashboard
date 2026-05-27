#include "gh_heatmap.hpp"
#include "kindle_personal_dashboard_core.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <gdk/gdk.h>
#include <glibconfig.h>
#include <gtk/gtk.h>
#include <memory>

namespace kpd {

GHHeatmap::GHHeatmap(std::shared_ptr<core::State> state)
	: core_state(std::move(state)), drawing_area(gtk_drawing_area_new()) {
	g_signal_connect(
		G_OBJECT(this->drawing_area), "expose-event",
		G_CALLBACK(_drawing_area_on_expose), this
	);
}

gboolean GHHeatmap::_drawing_area_on_expose(
	GtkWidget* widget,
	[[maybe_unused]] GdkEventExpose* event,
	gpointer user_data
) {
	auto* thiz = static_cast<GHHeatmap*>(user_data);
	const gint w = widget->allocation.width;
	const gint h = widget->allocation.height;

	const uintptr_t contribution_week_count =
		core::kpd_core_state_get_gh_heatmap_contribution_week_count(
			thiz->core_state.get()
		);

	GdkGC* gc = gdk_gc_new(widget->window);

	GdkColor background_color;
	gdk_color_parse("#0d1117", &background_color);
	gdk_gc_set_rgb_fg_color(gc, &background_color);
	gdk_draw_rectangle(widget->window, gc, TRUE, 0, 0, w, h);

	std::array<GdkColor, 5> level_colors{};
	gdk_color_parse("#151b23", &level_colors.at(0));
	gdk_color_parse("#033a16", &level_colors.at(1));
	gdk_color_parse("#196c2e", &level_colors.at(2));
	gdk_color_parse("#2ea043", &level_colors.at(3));
	gdk_color_parse("#56d364", &level_colors.at(4));

	const size_t square_size = static_cast<size_t>(w) / contribution_week_count;

	for (uintptr_t week = 0; week < contribution_week_count; week++) {
		uintptr_t level_count = 0;
		const uint8_t* levels = nullptr;
		core::kpd_core_state_get_gh_heatmap_contribution_week_levels(
			thiz->core_state.get(), week, &levels, &level_count
		);

		const size_t x = week * square_size;

		for (size_t weekday = 0; weekday < level_count; weekday++) {
			// NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
			const uint8_t level = levels[weekday];

			const size_t y = weekday * square_size;
			gdk_gc_set_rgb_fg_color(
				gc,
				&level_colors.at(
					static_cast<size_t>(std::min(level, MAX_CONTRIBUTION_LEVEL))
				)
			);
			gdk_draw_rectangle(
				widget->window, gc, TRUE, static_cast<gint>((2 * PADDING) + x),
				static_cast<gint>((2 * PADDING) + y),
				static_cast<gint>(square_size - PADDING),
				static_cast<gint>(square_size - PADDING)
			);
		}
	}

	g_object_unref(gc);

	return FALSE;
}

void GHHeatmap::configure_layout(GdkWindow* window) {
	gint win_w = 0;
	gint win_h = 0;
	gtk_window_get_size(GTK_WINDOW(window), &win_w, &win_h);

	const uintptr_t contribution_week_count =
		core::kpd_core_state_get_gh_heatmap_contribution_week_count(
			this->core_state.get()
		);

	const auto aspect_ratio =
		static_cast<float>(contribution_week_count) / 7.0f;

	gint want_h = static_cast<gint>(static_cast<float>(win_w) / aspect_ratio);
	want_h = std::max(want_h, 1);

	gtk_widget_set_size_request(this->drawing_area, -1, want_h);
	gtk_widget_queue_draw(this->drawing_area);
}

} // namespace kpd
