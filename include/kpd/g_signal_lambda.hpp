#pragma once

#include <functional>
#include <glib-object.h>
#include <gtk/gtk.h>
#include <utility>

namespace kpd {
inline static void on_signal_clicked_lambda(
	[[maybe_unused]] GtkWidget* widget,
	gpointer user_data
) {
	auto* lambda_ptr = static_cast<std::function<void()>*>(user_data);
	(*lambda_ptr)();
}

inline static void
delete_lambda(gpointer user_data, [[maybe_unused]] GClosure* closure) {
	auto* lambda_ptr = static_cast<std::function<void()>*>(user_data);
	delete lambda_ptr; // NOLINT(cppcoreguidelines-owning-memory)
}

// TODO: investigate how dangerous it is to ignore the lifetime of the lambda
inline static gulong g_signal_connect_clicked_lambda(
	GtkWidget* instance,
	std::function<void()> lambda
) {
	gpointer data = new std::function<void()>(std::move(lambda));

	// NOLINTBEGIN(clang-analyzer-optin.core.EnumCastOutOfRange)
	constexpr static auto default_flag =
		static_cast<GConnectFlags>(0 /* G_CONNECT_DEFAULT */);
	// NOLINTEND(clang-analyzer-optin.core.EnumCastOutOfRange)

	return g_signal_connect_data(
		instance, "clicked", G_CALLBACK(on_signal_clicked_lambda), data,
		delete_lambda, default_flag
	);
}
} // namespace kpd
