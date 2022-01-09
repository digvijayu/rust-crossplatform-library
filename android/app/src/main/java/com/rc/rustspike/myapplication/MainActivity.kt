package com.rc.rustspike.myapplication

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import android.widget.TextView

class MainActivity : AppCompatActivity(), JNICallback {
    init {
        System.loadLibrary("helloandroid")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        invokeCallbackViaJNI(this)
    }

    /**
     * A native method that is implemented by the 'rust' native library,
     * which is packaged with this application.
     */
    external fun invokeCallbackViaJNI(callback: JNICallback)

    override fun callback(string: String) {
        val textView = findViewById<View>(R.id.helloLabel) as TextView
        textView.setText("From JNI: $string\n")
    }
}