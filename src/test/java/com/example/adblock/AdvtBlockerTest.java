package com.example.adblock;

import java.util.ArrayList;
import java.util.List;

import static org.junit.Assert.assertTrue;
import org.junit.Test;

public class AdvtBlockerTest {

    private List<String> rules = new ArrayList<>(List.of(
        "-advertisement-icon.",
        "-advertisement-management/",
        "-advertisement.",
        "-advertisement/script."
    ));

    @Test
    public void createInstanceTest() {
        AdvtBlocker blocker = AdvtBlocker.createInstance(rules);
        assertTrue(blocker != null);
    }

    @Test
    public void checkTest() {
        AdvtBlocker blocker = AdvtBlocker.createInstance(rules);
        boolean result = blocker.checkUrls(
            "http://example.com/-advertisement-icon.",
            "http://example.com/helloworld",
            "image"
        );

        assertTrue(result);
    }
}
