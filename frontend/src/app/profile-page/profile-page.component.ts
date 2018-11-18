import { Component, OnInit } from "@angular/core";
import { MatSnackBar } from "@angular/material";

@Component({
  selector: "profile-page",
  templateUrl: "./profile-page.component.html",
  styleUrls: ["./profile-page.component.css"]
})
export class ProfilePageComponent implements OnInit {
  constructor(private snackBar: MatSnackBar) {}

  ngOnInit() {}

  onProfileUpdateSuccess() {
    this.snackBar.open("Profile update success");
  }

  onProfileUpdateFailure() {
    this.snackBar.open("Profile update failure");
  }
}
