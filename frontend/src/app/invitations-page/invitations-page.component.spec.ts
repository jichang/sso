import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { InvitationsPageComponent } from './invitations-page.component';

describe('InvitationsPageComponent', () => {
  let component: InvitationsPageComponent;
  let fixture: ComponentFixture<InvitationsPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ InvitationsPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(InvitationsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
