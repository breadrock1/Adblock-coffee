package com.example.simple;

import java.util.ArrayList;
import java.util.List;

import static org.junit.Assert.assertTrue;

import org.junit.Test;

import com.example.adblock.AdvtBlocker;

/**
 * Simple test of using library
 */
public class AppTest {
    /**
     * Rigorous Test :-)
     */
    @Test
    public void mainTest() {
        List<String> rules = new ArrayList<>(List.of(
            "-advertisement-icon.",
            "-advertisement-management/",
            "-advertisement.",
            "-advertisement/script."
        ));

        AdvtBlocker blocker = AdvtBlocker.createInstance(rules);
        boolean result = blocker.checkUrls(
            "http://example.com/-advertisement-icon.",
            "http://example.com/helloworld",
            "image"
        );

        assertTrue(result);
    }
}
