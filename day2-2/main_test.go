package main

import "testing"

func TestValid(t *testing.T) {
	tests := []struct {
		entry entry
		want  bool
	}{
		{
			entry: entry{
				policy:   policy{1, 3, 'a'},
				password: "abcde",
			},
			want: true,
		},
		{
			entry: entry{
				policy:   policy{1, 3, 'b'},
				password: "cdefg",
			},
			want: false,
		},
		{
			entry: entry{
				policy:   policy{1, 9, 'c'},
				password: "ccccccccc",
			},
			want: false,
		},
	}

	for _, test := range tests {
		got := test.entry.Valid()
		if got != test.want {
			t.Fatalf("got=%t want=%t: %#v", got, test.want, test.entry)
		}
	}
}
