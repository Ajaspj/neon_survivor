import wave
import math
import struct
import random

SAMPLE_RATE = 44100

def write_wav(filename, samples):
    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1) # mono
        wav_file.setsampwidth(2) # 16-bit
        wav_file.setframerate(SAMPLE_RATE)
        # Clamp and convert to 16-bit signed integer
        data = bytearray()
        for s in samples:
            clamped = max(-1.0, min(1.0, s))
            val = int(clamped * 32767.0)
            data.extend(struct.pack('<h', val))
        wav_file.writeframes(data)
    print(f"Generated {filename} ({len(samples)} samples, {len(samples)/SAMPLE_RATE:.2f}s)")

# 1. Laser shoot: Pitch drops from 950Hz to 180Hz over 0.14s
def gen_shoot():
    duration = 0.14
    total_samples = int(duration * SAMPLE_RATE)
    samples = []
    phase = 0.0
    for i in range(total_samples):
        t = i / total_samples
        # Exponential frequency drop
        freq = 950.0 * (0.18 ** t)
        phase += 2.0 * math.pi * freq / SAMPLE_RATE
        # Envelope: sharp attack, linear fade
        env = (1.0 - t) ** 1.5
        # Sine with slight square wave harmonic
        val = 0.8 * math.sin(phase) + 0.2 * math.sin(phase * 2.0)
        samples.append(val * env * 0.7)
    return samples

# 2. Enemy death explosion: Boom + decaying white noise
def gen_death():
    duration = 0.38
    total_samples = int(duration * SAMPLE_RATE)
    samples = []
    phase = 0.0
    rand = random.Random(42)
    for i in range(total_samples):
        t = i / total_samples
        freq = 140.0 * (1.0 - t * 0.7)
        phase += 2.0 * math.pi * freq / SAMPLE_RATE
        env = (1.0 - t) ** 2.2
        noise = (rand.random() * 2.0 - 1.0) * (1.0 - t)
        sub = math.sin(phase) * 0.6
        val = (noise * 0.5 + sub * 0.5) * env
        samples.append(val * 0.85)
    return samples

# 3. Player hurt: Distorted impact buzz descending
def gen_hurt():
    duration = 0.22
    total_samples = int(duration * SAMPLE_RATE)
    samples = []
    phase = 0.0
    for i in range(total_samples):
        t = i / total_samples
        freq = 240.0 * (1.0 - t * 0.6)
        phase += 2.0 * math.pi * freq / SAMPLE_RATE
        env = (1.0 - t) ** 1.8
        # Sawtooth / square buzz
        saw = 2.0 * (phase / (2.0 * math.pi) % 1.0) - 1.0
        samples.append(saw * env * 0.65)
    return samples

# 4. Level up: Ascending arpeggio C5 (523), E5 (659), G5 (784), C6 (1046)
def gen_levelup():
    duration = 0.55
    total_samples = int(duration * SAMPLE_RATE)
    notes = [523.25, 659.25, 783.99, 1046.50]
    note_dur = duration / len(notes)
    samples = []
    phase = 0.0
    for i in range(total_samples):
        t = i / SAMPLE_RATE
        note_idx = min(len(notes) - 1, int(t / note_dur))
        freq = notes[note_idx]
        local_t = (t - note_idx * note_dur) / note_dur
        phase += 2.0 * math.pi * freq / SAMPLE_RATE
        env = (1.0 - local_t * 0.8)
        # Rich synth tone
        val = 0.7 * math.sin(phase) + 0.3 * math.sin(phase * 2.0)
        samples.append(val * env * 0.7)
    return samples

# 5. XP pickup: High crisp ping
def gen_xp():
    duration = 0.12
    total_samples = int(duration * SAMPLE_RATE)
    samples = []
    phase = 0.0
    for i in range(total_samples):
        t = i / total_samples
        freq = 1350.0 + 300.0 * (1.0 - t)
        phase += 2.0 * math.pi * freq / SAMPLE_RATE
        env = (1.0 - t) ** 3.0
        val = math.sin(phase) * env
        samples.append(val * 0.6)
    return samples

# 6. Ambient Synth BGM Loop: 4-second seamless loop at 120 BPM
def gen_bgm():
    duration = 4.0 # 2 bars at 120 BPM
    total_samples = int(duration * SAMPLE_RATE)
    samples = [0.0] * total_samples
    bpm = 120.0
    beat_dur = 60.0 / bpm # 0.5 sec
    
    # Kick on every beat (0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5)
    for beat in range(8):
        start_sample = int(beat * beat_dur * SAMPLE_RATE)
        kick_samples = int(0.22 * SAMPLE_RATE)
        phase = 0.0
        for i in range(kick_samples):
            idx = start_sample + i
            if idx >= total_samples:
                break
            t = i / kick_samples
            freq = 150.0 * (0.08 ** t)
            phase += 2.0 * math.pi * freq / SAMPLE_RATE
            env = (1.0 - t) ** 2.5
            samples[idx] += math.sin(phase) * env * 0.55

    # 16th-note bassline (synth pulsing bass in Am: A, A, C, D, E, D, C, B...)
    bass_notes = [110.0, 110.0, 130.81, 146.83, 164.81, 146.83, 130.81, 123.47] * 2
    step_dur = duration / len(bass_notes) # 0.25 sec each
    for step, freq in enumerate(bass_notes):
        start_sample = int(step * step_dur * SAMPLE_RATE)
        step_samples = int(0.20 * SAMPLE_RATE)
        phase = 0.0
        for i in range(step_samples):
            idx = start_sample + i
            if idx >= total_samples:
                break
            t = i / step_samples
            phase += 2.0 * math.pi * freq / SAMPLE_RATE
            env = (1.0 - t) ** 1.6
            # Sub + warm saw
            saw = 2.0 * (phase / (2.0 * math.pi) % 1.0) - 1.0
            sub = math.sin(phase)
            samples[idx] += (0.6 * sub + 0.4 * saw) * env * 0.35

    # Hi-hat ticks on off-beats
    rand = random.Random(99)
    for step in range(16):
        if step % 2 == 1:
            start_sample = int((step * duration / 16.0) * SAMPLE_RATE)
            hat_len = int(0.04 * SAMPLE_RATE)
            for i in range(hat_len):
                idx = start_sample + i
                if idx >= total_samples:
                    break
                t = i / hat_len
                samples[idx] += (rand.random() * 2.0 - 1.0) * ((1.0 - t) ** 4) * 0.15

    return samples

if __name__ == '__main__':
    write_wav("assets/audio/shoot.wav", gen_shoot())
    write_wav("assets/audio/enemy_death.wav", gen_death())
    write_wav("assets/audio/player_hurt.wav", gen_hurt())
    write_wav("assets/audio/levelup.wav", gen_levelup())
    write_wav("assets/audio/xp_pickup.wav", gen_xp())
    write_wav("assets/audio/bgm.wav", gen_bgm())
    print("All audio files successfully generated.")
