let audioCtx = null;

function getAudioContext() {
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    }
    if (audioCtx.state === 'suspended') {
        audioCtx.resume();
    }
    return audioCtx;
}

window.playSwitchSound = function(type) {
    try {
        const ctx = getAudioContext();
        const now = ctx.currentTime;
        
        if (type === 'linear') {
            // Thocky sound (Yellow Switch)
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            const filter = ctx.createBiquadFilter();
            
            osc.type = 'sine';
            osc.frequency.setValueAtTime(140, now);
            osc.frequency.exponentialRampToValueAtTime(70, now + 0.1);
            
            gain.gain.setValueAtTime(0.6, now);
            gain.gain.exponentialRampToValueAtTime(0.01, now + 0.1);
            
            filter.type = 'lowpass';
            filter.frequency.setValueAtTime(280, now);
            
            osc.connect(filter);
            filter.connect(gain);
            gain.connect(ctx.destination);
            
            osc.start(now);
            osc.stop(now + 0.12);
        } else if (type === 'tactile') {
            // Creamy sound with small bump (Brown Switch)
            const osc1 = ctx.createOscillator();
            const osc2 = ctx.createOscillator();
            const gain1 = ctx.createGain();
            const gain2 = ctx.createGain();
            
            // Click/bump
            osc1.type = 'triangle';
            osc1.frequency.setValueAtTime(500, now);
            osc1.frequency.exponentialRampToValueAtTime(180, now + 0.03);
            gain1.gain.setValueAtTime(0.2, now);
            gain1.gain.exponentialRampToValueAtTime(0.01, now + 0.03);
            
            osc1.connect(gain1);
            gain1.connect(ctx.destination);
            osc1.start(now);
            osc1.stop(now + 0.04);
            
            // Release/Thock
            osc2.type = 'sine';
            osc2.frequency.setValueAtTime(120, now + 0.01);
            osc2.frequency.exponentialRampToValueAtTime(60, now + 0.09);
            gain2.gain.setValueAtTime(0.4, now + 0.01);
            gain2.gain.exponentialRampToValueAtTime(0.01, now + 0.09);
            
            osc2.connect(gain2);
            gain2.connect(ctx.destination);
            osc2.start(now + 0.01);
            osc2.stop(now + 0.1);
        } else if (type === 'clicky') {
            // Crisp click (Blue Switch)
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            
            osc.type = 'triangle';
            osc.frequency.setValueAtTime(900, now);
            osc.frequency.exponentialRampToValueAtTime(250, now + 0.04);
            
            gain.gain.setValueAtTime(0.3, now);
            gain.gain.exponentialRampToValueAtTime(0.01, now + 0.04);
            
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.start(now);
            osc.stop(now + 0.05);
            
            // White Noise click
            const bufferSize = ctx.sampleRate * 0.015;
            const buffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
            const data = buffer.getChannelData(0);
            for (let i = 0; i < bufferSize; i++) {
                data[i] = Math.random() * 2 - 1;
            }
            
            const noise = ctx.createBufferSource();
            noise.buffer = buffer;
            
            const noiseFilter = ctx.createBiquadFilter();
            noiseFilter.type = 'highpass';
            noiseFilter.frequency.setValueAtTime(3200, now);
            
            const noiseGain = ctx.createGain();
            noiseGain.gain.setValueAtTime(0.25, now);
            noiseGain.gain.exponentialRampToValueAtTime(0.01, now + 0.015);
            
            noise.connect(noiseFilter);
            noiseFilter.connect(noiseGain);
            noiseGain.connect(ctx.destination);
            
            noise.start(now);
            noise.stop(now + 0.02);
        }
    } catch (e) {
        console.error("Web Audio API not supported or user interaction required:", e);
    }
}
