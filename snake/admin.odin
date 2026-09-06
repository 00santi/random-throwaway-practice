package snake 
import rl "vendor:raylib"

init :: proc() {
	rl.InitWindow(WINDOW_SIZE, WINDOW_SIZE, "Snake")
	rl.InitAudioDevice()
	rl.SetTargetFPS(60)
	food_sprite = rl.LoadTexture("assets/food.png")
	head_sprite = rl.LoadTexture("assets/head.png")
	tail_sprite = rl.LoadTexture("assets/tail.png")
	body_sprite = rl.LoadTexture("assets/body.png")
	eat_sound = rl.LoadSound("assets/eat.wav")
	crash_sound = rl.LoadSound("assets/crash.wav")
	restart()
}

deinit :: proc() {
	rl.UnloadTexture(food_sprite)
	rl.UnloadTexture(head_sprite)
	rl.UnloadTexture(tail_sprite)
	rl.UnloadTexture(body_sprite)
	rl.UnloadSound(eat_sound)
	rl.UnloadSound(crash_sound)
	rl.CloseAudioDevice()
	rl.CloseWindow()
}