#pragma once

#define KINDLE_AWM_LAYER_PREFIX "L"
#define KINDLE_AWM_LAYER_APP "A"

#define KINDLE_AWM_NAME_PREFIX "N"
#define KINDLE_AWM_NAME_APPLICATION "application"

#define KINDLE_AWM_ID_PREFIX "ID"

#define KINDLE_AWM_PC_PREFIX "PC"
#define KINDLE_AWM_PC_TOPBAR_STATUS "T"

#define KINDLE_AWM_TITLE(LAYER, NAME, ID, PC)                                  \
	KINDLE_AWM_LAYER_PREFIX ":" LAYER "_" KINDLE_AWM_NAME_PREFIX ":" NAME      \
							"_" KINDLE_AWM_ID_PREFIX ":" ID                    \
							"_" KINDLE_AWM_PC_PREFIX ":" PC
