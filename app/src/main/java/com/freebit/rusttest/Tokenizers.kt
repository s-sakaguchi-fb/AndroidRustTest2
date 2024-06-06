package com.freebit.rusttest

class Tokenizers {
    companion object {
        init {
            System.loadLibrary("tokenizersjni")
        }
        @JvmStatic
        external fun serialization(filename: String)
    }
}