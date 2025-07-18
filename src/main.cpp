#include <Geode/Geode.hpp>
#include <Geode/modify/CCEGLView.hpp>
#include <Geode/modify/GJBaseGameLayer.hpp>
#include <Geode/modify/LevelEditorLayer.hpp>
#include <Geode/modify/PlayLayer.hpp>
#include <Geode/modify/PlayerObject.hpp>

using namespace geode::prelude;

#pragma comment(lib, "kernel32.lib")
#pragma comment(lib, "bcrypt.lib")
#pragma comment(lib, "advapi32.lib")
#pragma comment(lib, "legacy_stdio_definitions.lib")
#pragma comment(lib, "advapi32.lib")
#pragma comment(lib, "cfgmgr32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "kernel32.lib")
#pragma comment(lib, "msimg32.lib")
#pragma comment(lib, "opengl32.lib")
#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "shlwapi.lib")
#pragma comment(lib, "user32.lib")
#pragma comment(lib, "winspool.lib")
#pragma comment(lib, "kernel32.lib")
#pragma comment(lib, "advapi32.lib")
#pragma comment(lib, "kernel32.lib")
#pragma comment(lib, "ntdll.lib")
#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "kernel32.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "kernel32.lib")

extern "C" {
void DcD_on_wgl_swap_buffers(HDC hdc);
void DcD_initialize();
void DcD_uninitialize();
void DcD_on_action(uint8_t button, bool player2, bool push);
void DcD_on_reset();
void DcD_set_is_in_level(bool is_in_level);
void DcD_set_playlayer_time(double time);
void DcD_on_init(PlayLayer* playlayer);
void DcD_on_quit();
void DcD_on_death();
bool DcD_do_force_player2_sounds();
bool DcD_do_use_alternate_hook();
void DcD_on_update(float dt);
}

class GlHook {
public:
    bool m_inited = false;
    HDC m_deviceContext;

    void setup(CCEGLView* view) {
        if (m_inited)
            return;

        auto* glfwWindow = view->getWindow();
        m_deviceContext = *reinterpret_cast<HDC*>(
            reinterpret_cast<uintptr_t>(glfwWindow) + 632);
        m_inited = true;
    }
};

class $modify(CCEGLView) {
    void swapBuffers() {
        static GlHook glHook = GlHook();
        glHook.setup(this);

        DcD_on_wgl_swap_buffers(glHook.m_deviceContext);
        CCEGLView::swapBuffers();
    }
};

void onUnload() {
    DcD_uninitialize();
}

$on_mod(Loaded) {
    DcD_initialize();
    std::atexit(onUnload);
}

template <class R, class T> inline R& from(T base, intptr_t offset) {
    return *reinterpret_cast<R*>(reinterpret_cast<uintptr_t>(base) + offset);
}

inline double getTime() {
    auto playLayer = PlayLayer::get();
    return playLayer ? from<double>(playLayer, 968) : 0.0;
}

void handleAction(int button, bool player1, bool push, PlayLayer* playLayer) {
    DcD_on_action(static_cast<uint8_t>(button),
                      !player1 && playLayer &&
                          (playLayer->m_levelSettings->m_twoPlayerMode ||
                           DcD_do_force_player2_sounds()),
                      push);
}

class $modify(PlayerObject) {
	void handlePushOrRelease(PlayerButton button, bool push) {
		auto playLayer = PlayLayer::get();
		if (playLayer == nullptr && LevelEditorLayer::get() == nullptr) {
			DcD_set_is_in_level(false);
			return;
		}
		if ((button == PlayerButton::Left || button == PlayerButton::Right) && !this->m_isPlatformer) {
			return;
		}

		DcD_set_is_in_level(true);
		DcD_set_playlayer_time(getTime());

		bool player1 = playLayer && this == playLayer->m_player1;
		handleAction(static_cast<int>(button), player1, push, playLayer);
	}

	bool pushButton(PlayerButton button) {
		if (DcD_do_use_alternate_hook()) {
			handlePushOrRelease(button, true);
		}
		return PlayerObject::pushButton(button);
	}

	bool releaseButton(PlayerButton button) {
		if (DcD_do_use_alternate_hook()) {
			handlePushOrRelease(button, false);
		}
		return PlayerObject::releaseButton(button);
	}
};

class $modify(GJBaseGameLayer) {
	void handleButton(bool push, int button, bool player1) {
		if (DcD_do_use_alternate_hook()) {
			GJBaseGameLayer::handleButton(push, button, player1);
			return;
		}
		DcD_set_is_in_level(true);
		DcD_set_playlayer_time(getTime());

		auto playLayer = PlayLayer::get();
		bool is_invalid = playLayer && ((button == 2 || button == 3)
                        && !(player1 && playLayer->m_player1->m_isPlatformer)
                        && !(!player1 && playLayer->m_player2->m_isPlatformer));
		if (!is_invalid) {
			handleAction(button, player1, push, playLayer);
		}
		
		GJBaseGameLayer::handleButton(push, button, player1);
	}

	void update(float dt) {
		DcD_on_update(dt);
		GJBaseGameLayer::update(dt);
		DcD_set_playlayer_time(getTime());
	}

	bool init() {
		return GJBaseGameLayer::init();
	}
};

class $modify(PlayLayer) {
	void onQuit() {
		DcD_on_quit();
		PlayLayer::onQuit();
	}

	void resetLevel() {
		DcD_on_reset();
		PlayLayer::resetLevel();
	}

	void destroyPlayer(PlayerObject* player, GameObject* hit) {
		PlayLayer::destroyPlayer(player, hit);
		if (player->m_isDead) {
			DcD_on_death();
		}
	}
};

class $modify(LevelEditorLayer) {
	bool init(GJGameLevel* level, bool something) {
		DcD_on_init(nullptr);
		return LevelEditorLayer::init(level, something);
	}
};
