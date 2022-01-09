//
//  ContentView.swift
//  ios
//
//  Created by Digvijay Upadhyay on 09/01/2022.
//

import SwiftUI

struct ContentView: View {
    @State private var text = "Tap me to view rust message"
    
    var body: some View {
        Text(text).padding().onTapGesture {
            handleOnTap()
        }
    }
    
    func handleOnTap() {
        let result = rust_hello("")
        let swift_result = String(cString: result!)
        text = swift_result
        rust_hello_free(UnsafeMutablePointer(mutating: result))
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
