package snake
import rl "vendor:raylib"
import "core:fmt"

snake: [dynamic; MAX_SNAKE_SIZE]Vec2i
move_direction: Vec2i
tick_timer: f32
game_over: bool
food: Vec2i
occupied_cells: [NO_CELLS][NO_CELLS]bool
free_cells: [dynamic; MAX_SNAKE_SIZE]Vec2i
food_sprite: rl.Texture2D
head_sprite: rl.Texture2D
tail_sprite: rl.Texture2D
body_sprite: rl.Texture2D
eat_sound: rl.Sound
crash_sound: rl.Sound
score: i32

restart :: proc() {
	clear(&snake)
	move_direction = DOWN
	tick_timer = TICK_RATE
	game_over = false
	food = {2, 2}
	score = 0
	init_snake()
}

main :: proc() {
	init()
	
	for !rl.WindowShouldClose() {
		if game_over {
			if rl.IsKeyPressed(.ENTER) || rl.IsKeyPressed(.SPACE) do restart()
		}
		else {
			update()
		}
		
		rl.BeginDrawing()
		camera := get_camera()
		rl.BeginMode2D(camera)
		rl.ClearBackground(rl.Color{76, 53, 83, 255})

		draw_food()
		draw_snake_square()
		if game_over do draw_game_over_text()
		else do draw_score()
		
		rl.EndMode2D()
		rl.EndDrawing()
		free_all(context.temp_allocator)
	}

	deinit()
}

update :: proc() {
	move_direction = get_direction()
	tick_timer -= rl.GetFrameTime()
	if tick_timer <= 0 {
		update_snake()
		check_for_game_over()
		tick_timer += TICK_RATE
	}
}

place_new_food :: proc() {
	occupied_cells = {}
	clear(&free_cells)
	for part in snake {
		occupied_cells[part.x][part.y] = true
	}

	for x in i32(0)..<NO_CELLS {
		for y in i32(0)..<NO_CELLS {
			if !occupied_cells[x][y] do append(&free_cells, Vec2i{x, y})
		}
	}

	if len(free_cells) > 0 {
		idx := rl.GetRandomValue(0, i32(len(free_cells) - 1))
		food = free_cells[idx]
	}
}

draw_food :: proc() {
	rl.DrawTexture(food_sprite, food.x * CELL_SIZE, food.y * CELL_SIZE, rl.WHITE)
}

draw_food_square :: proc() {
	rect := rl.Rectangle {
		f32(food.x) * CELL_SIZE,
		f32(food.y) * CELL_SIZE,
		CELL_SIZE,
		CELL_SIZE,
	}
	rl.DrawRectangleRec(rect, rl.GREEN)
}

draw_snake :: proc() {
	for part, idx in snake {
		sprite := idx == 0 ? head_sprite : (idx == len(snake) - 1 ? tail_sprite : body_sprite) 
		rl.DrawTexture(sprite, snake[idx].x * CELL_SIZE, snake[idx].y * CELL_SIZE, rl.WHITE)
	}
}

draw_snake_square :: proc() {
	for body_part in snake {
		rect := rl.Rectangle {
			f32(body_part.x) * CELL_SIZE,
			f32(body_part.y) * CELL_SIZE,
			CELL_SIZE,
			CELL_SIZE,
		}
		rl.DrawRectangleRec(rect, rl.RAYWHITE)
	}
}

draw_score :: proc() {
	text := fmt.ctprintf("Score: %v", score)
	rl.DrawText(text, 4, CANVAS_SIZE - 14, 10, rl.GRAY)
}

draw_game_over_text :: proc() {
	rl.DrawText("Game Over!", 4, 4, 15, rl.RED)
	rl.DrawText("Press ENTER or SPACE to restart", 4, 20, 7, rl.BLACK)
}

get_camera :: proc() -> rl.Camera2D {
	return rl.Camera2D {
		offset = {},
		rotation = {},
		target = {}, 
		zoom = CAMERA_ZOOM,
	}
}

get_direction :: proc() -> Vec2i {
	dir := move_direction

	up := rl.IsKeyDown(rl.KeyboardKey.UP) || rl.IsKeyDown(.W) || rl.IsKeyDown(.K)
	down := rl.IsKeyDown(.DOWN) || rl.IsKeyDown(.S) || rl.IsKeyDown(.J)
	left := rl.IsKeyDown(.LEFT) || rl.IsKeyDown(.A) || rl.IsKeyDown(.H)
	right := rl.IsKeyDown(.RIGHT) || rl.IsKeyDown(.D) || rl.IsKeyDown(.L)

	if up && !down do dir = UP
	else if down && !up do dir = DOWN

	if left && !right do dir = LEFT
	else if right && !left do dir = RIGHT
	
	return dir
}

init_snake :: proc() {
	assert(len(snake) == 0, "snake should be empty")
	head :: Vec2i { NO_CELLS / 2, NO_CELLS / 2 }
	snake_append(head)
	snake_append(snake_last() + UP)
	snake_append(snake_last() + UP)
}

snake_last :: proc() -> Vec2i {
	return snake[len(snake) - 1]
}

snake_append :: proc(v: Vec2i) {
	append(&snake, v)
}

update_snake :: proc() {
	last := snake[0]
	snake[0] += move_direction
	for i in 1..<len(snake) {
		if snake[i] == snake[0] {
			game_over = true
			rl.PlaySound(crash_sound)
		}
		else do snake[i], last = last, snake[i]
	}
	if snake[0] == food {
		score += 1
		rl.PlaySound(eat_sound)
		snake_append(last)
		place_new_food()
	}
}

check_for_game_over :: proc() {
	head := snake[0]
	if head.x >= NO_CELLS || head.y >= NO_CELLS || head.x < 0 || head.y < 0 {
		game_over = true
		rl.PlaySound(crash_sound)
	}
}
